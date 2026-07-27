# Configuration Management Strategies in .NET

## Metadata

- **Difficulty Level**: L3 (Advanced)
- **Category**: .NET Architecture
- **Prerequisites**: ASP.NET Core basics, dependency injection, JSON/XML understanding
- **Learning Objectives**:
  - Master different configuration sources and their precedence
  - Implement secure configuration management
  - Understand configuration validation and binding
  - Design environment-specific configuration strategies
- **Estimated Study Time**: 50-65 minutes
- **Last Updated**: 2025-01-27

## Question

**"How do you manage configuration in .NET applications? What are the different sources and best practices for secure configuration management?"**

## Brief Answer

Configuration in .NET is managed through the **Configuration API** which supports multiple sources (appsettings.json, environment variables, command line, user secrets, Azure Key Vault) with a specific precedence order. Best practices include using strong typing with IOptions, securing sensitive data with user secrets or key vaults, implementing validation, and organizing configuration by environment with proper inheritance.

## Detailed Explanation

### Configuration Sources and Precedence

#### **Default Configuration Order (Highest to Lowest Priority):**

1. Command line arguments
2. Environment variables
3. User secrets (Development only)
4. appsettings.{Environment}.json
5. appsettings.json
6. Host-specific configuration

### Configuration File Structures

#### **1. Basic appsettings.json Structure:**

```json
{
  "Logging": {
    "LogLevel": {
      "Default": "Information",
      "Microsoft.AspNetCore": "Warning"
    }
  },
  "ConnectionStrings": {
    "DefaultConnection": "Server=(localdb)\\mssqllocaldb;Database=MyApp;Trusted_Connection=true;",
    "Redis": "localhost:6379"
  },
  "AppSettings": {
    "ApplicationName": "MyApplication",
    "Version": "1.0.0",
    "MaxRetryAttempts": 3,
    "TimeoutSeconds": 30
  },
  "ExternalServices": {
    "PaymentGateway": {
      "BaseUrl": "https://api.payment.com",
      "ApiVersion": "v2",
      "TimeoutMs": 5000
    },
    "EmailService": {
      "SmtpServer": "smtp.company.com",
      "Port": 587,
      "EnableSsl": true
    }
  },
  "Security": {
    "EncryptionKey": "placeholder-will-be-overridden",
    "JwtSettings": {
      "ValidIssuer": "https://myapp.com",
      "ValidAudience": "https://myapp.com",
      "ExpiryMinutes": 60
    }
  }
}
```

#### **2. Environment-Specific Configuration:**

**appsettings.Development.json:**

```json
{
  "Logging": {
    "LogLevel": {
      "Default": "Debug",
      "System": "Information",
      "Microsoft": "Information"
    }
  },
  "ConnectionStrings": {
    "DefaultConnection": "Server=(localdb)\\mssqllocaldb;Database=MyApp_Dev;Trusted_Connection=true;"
  },
  "ExternalServices": {
    "PaymentGateway": {
      "BaseUrl": "https://sandbox-api.payment.com"
    }
  }
}
```

**appsettings.Production.json:**

```json
{
  "Logging": {
    "LogLevel": {
      "Default": "Warning",
      "Microsoft.AspNetCore": "Error"
    }
  },
  "ExternalServices": {
    "PaymentGateway": {
      "BaseUrl": "https://api.payment.com",
      "TimeoutMs": 10000
    }
  }
}
```

### Strong-Typed Configuration with IOptions

#### **1. Configuration Classes:**

```csharp
public class AppSettings
{
    public const string SectionName = "AppSettings";
    
    public string ApplicationName { get; set; } = string.Empty;
    public string Version { get; set; } = string.Empty;
    public int MaxRetryAttempts { get; set; } = 3;
    public int TimeoutSeconds { get; set; } = 30;
}

public class ExternalServicesSettings
{
    public const string SectionName = "ExternalServices";
    
    public PaymentGatewaySettings PaymentGateway { get; set; } = new();
    public EmailServiceSettings EmailService { get; set; } = new();
}

public class PaymentGatewaySettings
{
    public string BaseUrl { get; set; } = string.Empty;
    public string ApiVersion { get; set; } = "v1";
    public int TimeoutMs { get; set; } = 5000;
    public string ApiKey { get; set; } = string.Empty; // Will come from secrets
}

public class EmailServiceSettings
{
    public string SmtpServer { get; set; } = string.Empty;
    public int Port { get; set; } = 587;
    public bool EnableSsl { get; set; } = true;
    public string Username { get; set; } = string.Empty;
    public string Password { get; set; } = string.Empty; // Will come from secrets
}

public class JwtSettings
{
    public const string SectionName = "Security:JwtSettings";
    
    public string ValidIssuer { get; set; } = string.Empty;
    public string ValidAudience { get; set; } = string.Empty;
    public int ExpiryMinutes { get; set; } = 60;
    public string SecretKey { get; set; } = string.Empty; // Will come from secrets
}
```

#### **2. Configuration Registration:**

```csharp
public class Program
{
    public static void Main(string[] args)
    {
        var builder = WebApplication.CreateBuilder(args);
        
        // Register configuration sections
        builder.Services.Configure<AppSettings>(
            builder.Configuration.GetSection(AppSettings.SectionName));
        
        builder.Services.Configure<ExternalServicesSettings>(
            builder.Configuration.GetSection(ExternalServicesSettings.SectionName));
        
        builder.Services.Configure<JwtSettings>(
            builder.Configuration.GetSection(JwtSettings.SectionName));
        
        // Register services that need configuration
        builder.Services.AddScoped<IPaymentService, PaymentService>();
        builder.Services.AddScoped<IEmailService, EmailService>();
        
        var app = builder.Build();
    }
}
```

#### **3. Configuration Usage in Services:**

```csharp
public class PaymentService : IPaymentService
{
    private readonly PaymentGatewaySettings _settings;
    private readonly HttpClient _httpClient;
    private readonly ILogger<PaymentService> _logger;
    
    public PaymentService(
        IOptions<ExternalServicesSettings> externalSettings,
        HttpClient httpClient,
        ILogger<PaymentService> logger)
    {
        _settings = externalSettings.Value.PaymentGateway;
        _httpClient = httpClient;
        _logger = logger;
        
        // Configure HttpClient
        _httpClient.BaseAddress = new Uri(_settings.BaseUrl);
        _httpClient.Timeout = TimeSpan.FromMilliseconds(_settings.TimeoutMs);
        _httpClient.DefaultRequestHeaders.Add("X-API-Version", _settings.ApiVersion);
        _httpClient.DefaultRequestHeaders.Add("Authorization", $"Bearer {_settings.ApiKey}");
    }
    
    public async Task<PaymentResult> ProcessPaymentAsync(PaymentRequest request)
    {
        try
        {
            _logger.LogInformation("Processing payment for amount {Amount}", request.Amount);
            
            var response = await _httpClient.PostAsJsonAsync("/payments", request);
            response.EnsureSuccessStatusCode();
            
            var result = await response.Content.ReadFromJsonAsync<PaymentResult>();
            return result ?? throw new InvalidOperationException("Invalid payment response");
        }
        catch (HttpRequestException ex)
        {
            _logger.LogError(ex, "Payment processing failed for amount {Amount}", request.Amount);
            throw new PaymentException("Payment processing failed", ex);
        }
    }
}

public class EmailService : IEmailService
{
    private readonly EmailServiceSettings _settings;
    private readonly ILogger<EmailService> _logger;
    
    public EmailService(
        IOptions<ExternalServicesSettings> externalSettings,
        ILogger<EmailService> logger)
    {
        _settings = externalSettings.Value.EmailService;
        _logger = logger;
    }
    
    public async Task SendEmailAsync(string to, string subject, string body)
    {
        try
        {
            using var client = new SmtpClient(_settings.SmtpServer, _settings.Port);
            client.EnableSsl = _settings.EnableSsl;
            
            if (!string.IsNullOrEmpty(_settings.Username))
            {
                client.Credentials = new NetworkCredential(_settings.Username, _settings.Password);
            }
            
            var message = new MailMessage("noreply@myapp.com", to, subject, body);
            await client.SendMailAsync(message);
            
            _logger.LogInformation("Email sent successfully to {Recipient}", to);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to send email to {Recipient}", to);
            throw;
        }
    }
}
```

### Secure Configuration Management

#### **1. User Secrets (Development):**

```bash
# Initialize user secrets
dotnet user-secrets init

# Add secrets
dotnet user-secrets set "ExternalServices:PaymentGateway:ApiKey" "dev-api-key-12345"
dotnet user-secrets set "ExternalServices:EmailService:Password" "dev-email-password"
dotnet user-secrets set "Security:JwtSettings:SecretKey" "dev-jwt-secret-key-very-long-and-secure"

# List secrets
dotnet user-secrets list
```

**secrets.json location:**

- Windows: `%APPDATA%\Microsoft\UserSecrets\<user_secrets_id>\secrets.json`
- macOS/Linux: `~/.microsoft/usersecrets/<user_secrets_id>/secrets.json`

#### **2. Environment Variables:**

```bash
# Windows (PowerShell)
$env:ExternalServices__PaymentGateway__ApiKey = "prod-api-key-67890"
$env:ExternalServices__EmailService__Password = "prod-email-password"
$env:Security__JwtSettings__SecretKey = "prod-jwt-secret-key-very-long-and-secure"

# Linux/macOS
export ExternalServices__PaymentGateway__ApiKey="prod-api-key-67890"
export ExternalServices__EmailService__Password="prod-email-password"
export Security__JwtSettings__SecretKey="prod-jwt-secret-key-very-long-and-secure"
```

#### **3. Azure Key Vault Integration:**

```csharp
public class Program
{
    public static void Main(string[] args)
    {
        var builder = WebApplication.CreateBuilder(args);
        
        // Add Azure Key Vault if in production
        if (builder.Environment.IsProduction())
        {
            var keyVaultEndpoint = builder.Configuration["KeyVaultSettings:Endpoint"];
            if (!string.IsNullOrEmpty(keyVaultEndpoint))
            {
                builder.Configuration.AddAzureKeyVault(
                    new Uri(keyVaultEndpoint),
                    new DefaultAzureCredential());
            }
        }
        
        var app = builder.Build();
    }
}
```

**Azure Key Vault setup:**

```json
{
  "KeyVaultSettings": {
    "Endpoint": "https://myapp-keyvault.vault.azure.net/"
  }
}
```

### Configuration Validation

#### **1. Data Annotations Validation:**

```csharp
public class PaymentGatewaySettings
{
    [Required]
    [Url]
    public string BaseUrl { get; set; } = string.Empty;
    
    [Required]
    [RegularExpression(@"^v\d+$", ErrorMessage = "API version must be in format 'v1', 'v2', etc.")]
    public string ApiVersion { get; set; } = "v1";
    
    [Range(1000, 60000, ErrorMessage = "Timeout must be between 1 and 60 seconds")]
    public int TimeoutMs { get; set; } = 5000;
    
    [Required(ErrorMessage = "API Key is required")]
    [MinLength(10, ErrorMessage = "API Key must be at least 10 characters")]
    public string ApiKey { get; set; } = string.Empty;
}

public class EmailServiceSettings
{
    [Required]
    public string SmtpServer { get; set; } = string.Empty;
    
    [Range(1, 65535)]
    public int Port { get; set; } = 587;
    
    public bool EnableSsl { get; set; } = true;
    
    [EmailAddress]
    public string Username { get; set; } = string.Empty;
    
    [Required(ErrorMessage = "Email password is required")]
    public string Password { get; set; } = string.Empty;
}
```

#### **2. Configuration Validation Setup:**

```csharp
public class Program
{
    public static void Main(string[] args)
    {
        var builder = WebApplication.CreateBuilder(args);
        
        // Register with validation
        builder.Services
            .AddOptions<PaymentGatewaySettings>()
            .Bind(builder.Configuration.GetSection("ExternalServices:PaymentGateway"))
            .ValidateDataAnnotations()
            .ValidateOnStart(); // Validate at startup
        
        builder.Services
            .AddOptions<EmailServiceSettings>()
            .Bind(builder.Configuration.GetSection("ExternalServices:EmailService"))
            .ValidateDataAnnotations()
            .Validate(settings =>
            {
                // Custom validation
                if (settings.EnableSsl && settings.Port == 25)
                {
                    return false; // SSL should not use port 25
                }
                return true;
            }, "SSL enabled but using insecure port 25")
            .ValidateOnStart();
        
        var app = builder.Build();
    }
}
```

#### **3. Custom Validation:**

```csharp
public class CustomConfigurationValidator : IValidateOptions<ExternalServicesSettings>
{
    public ValidateOptionsResult Validate(string? name, ExternalServicesSettings options)
    {
        var failures = new List<string>();
        
        // Validate PaymentGateway
        if (string.IsNullOrEmpty(options.PaymentGateway.ApiKey))
        {
            failures.Add("PaymentGateway ApiKey is required");
        }
        
        if (!Uri.TryCreate(options.PaymentGateway.BaseUrl, UriKind.Absolute, out var uri) || 
            uri.Scheme != "https")
        {
            failures.Add("PaymentGateway BaseUrl must be a valid HTTPS URL");
        }
        
        // Validate EmailService
        if (options.EmailService.EnableSsl && options.EmailService.Port == 25)
        {
            failures.Add("EmailService cannot use SSL with port 25");
        }
        
        return failures.Count > 0 
            ? ValidateOptionsResult.Fail(failures)
            : ValidateOptionsResult.Success;
    }
}

// Register custom validator
builder.Services.AddSingleton<IValidateOptions<ExternalServicesSettings>, CustomConfigurationValidator>();
```

### Advanced Configuration Patterns

#### **1. Configuration Reloading:**

```csharp
public class ConfigurationReloadService : BackgroundService
{
    private readonly IOptionsMonitor<AppSettings> _appSettingsMonitor;
    private readonly ILogger<ConfigurationReloadService> _logger;
    private IDisposable? _changeListener;
    
    public ConfigurationReloadService(
        IOptionsMonitor<AppSettings> appSettingsMonitor,
        ILogger<ConfigurationReloadService> logger)
    {
        _appSettingsMonitor = appSettingsMonitor;
        _logger = logger;
    }
    
    protected override Task ExecuteAsync(CancellationToken stoppingToken)
    {
        _changeListener = _appSettingsMonitor.OnChange(OnConfigurationChanged);
        return Task.CompletedTask;
    }
    
    private void OnConfigurationChanged(AppSettings newSettings, string? name)
    {
        _logger.LogInformation("Configuration changed. New MaxRetryAttempts: {MaxRetryAttempts}", 
            newSettings.MaxRetryAttempts);
        
        // React to configuration changes
        // Update caches, reconnect services, etc.
    }
    
    public override void Dispose()
    {
        _changeListener?.Dispose();
        base.Dispose();
    }
}

// Registration
builder.Services.AddHostedService<ConfigurationReloadService>();
```

#### **2. Configuration Factories:**

```csharp
public interface IHttpClientFactory<T>
{
    HttpClient CreateClient();
}

public class PaymentGatewayHttpClientFactory : IHttpClientFactory<PaymentService>
{
    private readonly IOptionsMonitor<ExternalServicesSettings> _optionsMonitor;
    
    public PaymentGatewayHttpClientFactory(IOptionsMonitor<ExternalServicesSettings> optionsMonitor)
    {
        _optionsMonitor = optionsMonitor;
    }
    
    public HttpClient CreateClient()
    {
        var settings = _optionsMonitor.CurrentValue.PaymentGateway;
        
        var client = new HttpClient
        {
            BaseAddress = new Uri(settings.BaseUrl),
            Timeout = TimeSpan.FromMilliseconds(settings.TimeoutMs)
        };
        
        client.DefaultRequestHeaders.Add("X-API-Version", settings.ApiVersion);
        client.DefaultRequestHeaders.Authorization = 
            new AuthenticationHeaderValue("Bearer", settings.ApiKey);
        
        return client;
    }
}

// Registration
builder.Services.AddScoped<IHttpClientFactory<PaymentService>, PaymentGatewayHttpClientFactory>();
```

#### **3. Configuration Binding with Complex Types:**

```csharp
public class DatabaseSettings
{
    public string ConnectionString { get; set; } = string.Empty;
    public int CommandTimeout { get; set; } = 30;
    public bool EnableRetryOnFailure { get; set; } = true;
    public RetrySettings Retry { get; set; } = new();
    public Dictionary<string, string> AdditionalParameters { get; set; } = new();
}

public class RetrySettings
{
    public int MaxRetryCount { get; set; } = 3;
    public TimeSpan MaxRetryDelay { get; set; } = TimeSpan.FromSeconds(30);
    public List<int> ErrorNumbersToAdd { get; set; } = new();
}

// appsettings.json
{
  "Database": {
    "ConnectionString": "Server=localhost;Database=MyApp;Trusted_Connection=true;",
    "CommandTimeout": 60,
    "EnableRetryOnFailure": true,
    "Retry": {
      "MaxRetryCount": 5,
      "MaxRetryDelay": "00:01:00",
      "ErrorNumbersToAdd": [2, 20, 64, 233, 10053, 10054, 10060, 40197, 40501, 40613]
    },
    "AdditionalParameters": {
      "MultipleActiveResultSets": "true",
      "TrustServerCertificate": "true"
    }
  }
}

// Registration and usage
builder.Services.Configure<DatabaseSettings>(builder.Configuration.GetSection("Database"));

public class DatabaseService
{
    public DatabaseService(IOptions<DatabaseSettings> dbSettings)
    {
        var settings = dbSettings.Value;
        
        var connectionStringBuilder = new SqlConnectionStringBuilder(settings.ConnectionString);
        
        // Apply additional parameters
        foreach (var param in settings.AdditionalParameters)
        {
            connectionStringBuilder[param.Key] = param.Value;
        }
        
        // Configure DbContext with retry policy
        var optionsBuilder = new DbContextOptionsBuilder<AppDbContext>();
        optionsBuilder.UseSqlServer(connectionStringBuilder.ConnectionString, options =>
        {
            if (settings.EnableRetryOnFailure)
            {
                options.EnableRetryOnFailure(
                    maxRetryCount: settings.Retry.MaxRetryCount,
                    maxRetryDelay: settings.Retry.MaxRetryDelay,
                    errorNumbersToAdd: settings.Retry.ErrorNumbersToAdd);
            }
            
            options.CommandTimeout(settings.CommandTimeout);
        });
    }
}
```

### Configuration Best Practices

#### **1. Hierarchical Configuration Structure:**

```csharp
public class ApplicationConfiguration
{
    public AppIdentity Identity { get; set; } = new();
    public SecuritySettings Security { get; set; } = new();
    public ExternalServicesSettings ExternalServices { get; set; } = new();
    public InfrastructureSettings Infrastructure { get; set; } = new();
    public FeatureFlags Features { get; set; } = new();
}

public class AppIdentity
{
    public string Name { get; set; } = string.Empty;
    public string Version { get; set; } = string.Empty;
    public string Environment { get; set; } = string.Empty;
}

public class SecuritySettings
{
    public AuthenticationSettings Authentication { get; set; } = new();
    public AuthorizationSettings Authorization { get; set; } = new();
    public EncryptionSettings Encryption { get; set; } = new();
}

public class InfrastructureSettings
{
    public DatabaseSettings Database { get; set; } = new();
    public CacheSettings Cache { get; set; } = new();
    public LoggingSettings Logging { get; set; } = new();
}

public class FeatureFlags
{
    public bool EnableNewPaymentFlow { get; set; } = false;
    public bool EnableAdvancedAnalytics { get; set; } = false;
    public bool EnableBetaFeatures { get; set; } = false;
}
```

#### **2. Configuration Extensions:**

```csharp
public static class ConfigurationExtensions
{
    public static IServiceCollection AddApplicationConfiguration(
        this IServiceCollection services, 
        IConfiguration configuration)
    {
        // Register all configuration sections
        services.Configure<ApplicationConfiguration>(configuration);
        services.Configure<AppIdentity>(configuration.GetSection("Identity"));
        services.Configure<SecuritySettings>(configuration.GetSection("Security"));
        services.Configure<ExternalServicesSettings>(configuration.GetSection("ExternalServices"));
        services.Configure<InfrastructureSettings>(configuration.GetSection("Infrastructure"));
        services.Configure<FeatureFlags>(configuration.GetSection("Features"));
        
        // Add validation
        services.AddSingleton<IValidateOptions<ApplicationConfiguration>, ApplicationConfigurationValidator>();
        
        return services;
    }
    
    public static string GetRequiredConnectionString(this IConfiguration configuration, string name)
    {
        var connectionString = configuration.GetConnectionString(name);
        if (string.IsNullOrEmpty(connectionString))
        {
            throw new InvalidOperationException($"Connection string '{name}' is required but not configured.");
        }
        return connectionString;
    }
    
    public static T GetRequiredSection<T>(this IConfiguration configuration, string sectionName) 
        where T : class, new()
    {
        var section = configuration.GetSection(sectionName);
        if (!section.Exists())
        {
            throw new InvalidOperationException($"Configuration section '{sectionName}' is required but not found.");
        }
        
        var value = section.Get<T>();
        return value ?? throw new InvalidOperationException($"Failed to bind configuration section '{sectionName}'.");
    }
}

// Usage
builder.Services.AddApplicationConfiguration(builder.Configuration);
```

## Security Considerations

### **1. Secrets Management:**

```csharp
public class SecureConfigurationService
{
    private readonly IConfiguration _configuration;
    private readonly IWebHostEnvironment _environment;
    
    public SecureConfigurationService(IConfiguration configuration, IWebHostEnvironment environment)
    {
        _configuration = configuration;
        _environment = environment;
    }
    
    public string GetSecureValue(string key)
    {
        // Priority: Key Vault > Environment Variables > User Secrets > Config Files
        var value = _configuration[key];
        
        if (string.IsNullOrEmpty(value))
        {
            throw new SecurityException($"Secure configuration value '{key}' not found in any secure source.");
        }
        
        // Log access (but not the value!)
        Logger.LogInformation("Accessed secure configuration: {Key}", key);
        
        return value;
    }
    
    public void ValidateSecureConfiguration()
    {
        var requiredSecrets = new[]
        {
            "Security:JwtSettings:SecretKey",
            "ExternalServices:PaymentGateway:ApiKey",
            "ExternalServices:EmailService:Password"
        };
        
        foreach (var secret in requiredSecrets)
        {
            var value = _configuration[secret];
            if (string.IsNullOrEmpty(value))
            {
                throw new SecurityException($"Required secret '{secret}' is not configured.");
            }
            
            if (_environment.IsProduction() && value.Contains("dev") || value.Contains("test"))
            {
                throw new SecurityException($"Secret '{secret}' contains development/test values in production.");
            }
        }
    }
}
```

### **2. Configuration Encryption:**

```csharp
public class EncryptedConfigurationProvider : ConfigurationProvider
{
    private readonly string _filePath;
    private readonly string _encryptionKey;
    
    public EncryptedConfigurationProvider(string filePath, string encryptionKey)
    {
        _filePath = filePath;
        _encryptionKey = encryptionKey;
    }
    
    public override void Load()
    {
        if (!File.Exists(_filePath))
        {
            return;
        }
        
        var encryptedContent = File.ReadAllText(_filePath);
        var decryptedContent = Decrypt(encryptedContent, _encryptionKey);
        var configData = JsonSerializer.Deserialize<Dictionary<string, string>>(decryptedContent);
        
        Data = configData ?? new Dictionary<string, string>();
    }
    
    private string Decrypt(string encryptedText, string key)
    {
        // Implementation depends on your encryption strategy
        // This is a simplified example
        using var aes = Aes.Create();
        aes.Key = Encoding.UTF8.GetBytes(key.PadRight(32).Substring(0, 32));
        aes.IV = new byte[16]; // Use proper IV in real implementation
        
        var encrypted = Convert.FromBase64String(encryptedText);
        using var decryptor = aes.CreateDecryptor();
        using var msDecrypt = new MemoryStream(encrypted);
        using var csDecrypt = new CryptoStream(msDecrypt, decryptor, CryptoStreamMode.Read);
        using var reader = new StreamReader(csDecrypt);
        
        return reader.ReadToEnd();
    }
}

public class EncryptedConfigurationSource : IConfigurationSource
{
    private readonly string _filePath;
    private readonly string _encryptionKey;
    
    public EncryptedConfigurationSource(string filePath, string encryptionKey)
    {
        _filePath = filePath;
        _encryptionKey = encryptionKey;
    }
    
    public IConfigurationProvider Build(IConfigurationBuilder builder)
    {
        return new EncryptedConfigurationProvider(_filePath, _encryptionKey);
    }
}

// Usage
builder.Configuration.Add(new EncryptedConfigurationSource("secrets.encrypted.json", encryptionKey));
```

## Performance Implications

### **Configuration Caching and Performance:**

```csharp
public class OptimizedConfigurationService
{
    private readonly IOptionsSnapshot<AppSettings> _appSettings;
    private readonly IMemoryCache _cache;
    private readonly ILogger<OptimizedConfigurationService> _logger;
    
    public OptimizedConfigurationService(
        IOptionsSnapshot<AppSettings> appSettings,
        IMemoryCache cache,
        ILogger<OptimizedConfigurationService> logger)
    {
        _appSettings = appSettings;
        _cache = cache;
        _logger = logger;
    }
    
    public T GetCachedConfiguration<T>(string cacheKey, Func<T> configurationFactory, TimeSpan? expiry = null)
    {
        return _cache.GetOrCreate(cacheKey, entry =>
        {
            entry.AbsoluteExpirationRelativeToNow = expiry ?? TimeSpan.FromMinutes(30);
            entry.Priority = CacheItemPriority.High;
            
            _logger.LogDebug("Loading configuration into cache: {CacheKey}", cacheKey);
            return configurationFactory();
        })!;
    }
}
```

## Related Topics

```plaintext
- [[Dependency Injection in .NET]]
- [[ASP.NET Core Middleware]]
- [[Azure Key Vault Integration]]
- [[Options Pattern in .NET]]
- [[Environment Variables Management]]
- [[Configuration Binding and Validation]]
- [[Secrets Management Best Practices]]
```

## Confidence Check

- [ ] I understand different configuration sources and their precedence
- [ ] I can implement strong-typed configuration with IOptions
- [ ] I understand secure configuration management practices
- [ ] I can implement configuration validation
- [ ] I can create custom configuration providers
- [ ] I understand environment-specific configuration strategies
- [ ] I can implement configuration reloading and monitoring

## Additional Resources

- [Microsoft Docs: Configuration in ASP.NET Core](https://docs.microsoft.com/en-us/aspnet/core/fundamentals/configuration/)
- [Options Pattern in ASP.NET Core](https://docs.microsoft.com/en-us/aspnet/core/fundamentals/configuration/options)
- [Azure Key Vault Configuration Provider](https://docs.microsoft.com/en-us/aspnet/core/security/key-vault-configuration)
- [Safe Storage of App Secrets](https://docs.microsoft.com/en-us/aspnet/core/security/app-secrets)
