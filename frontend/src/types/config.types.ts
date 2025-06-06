export interface ConfigData {
  // Server settings
  ip: string;
  port: number;
  enable_oai: boolean;

  // App settings
  check_update: boolean;
  auto_update: boolean;

  // Network settings
  password: string;
  admin_password: string;
  proxy: string | null;
  rproxy: string | null;

  // API settings
  max_retries: number;
  pass_params: boolean;
  preserve_chats: boolean;

  // Cookie settings
  skip_first_warning: boolean;
  skip_second_warning: boolean;
  skip_restricted: boolean;
  skip_non_pro: boolean;
  skip_rate_limit: boolean;
  skip_normal_pro: boolean;

  // Prompt configurations
  use_real_roles: boolean;
  custom_h: string | null;
  custom_a: string | null;
  custom_prompt: string;
  padtxt_file: string | null;
  padtxt_len: number;
}

export interface ConfigState {
  config: ConfigData | null;
  originalPassword: string;
  originalAdminPassword: string;
  loading: boolean;
  saving: boolean;
  error: string;
}
