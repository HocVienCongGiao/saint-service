module "saints" {
  source = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//skeleton/services/service-function"
  service_name = var.service_name
  query_api_parent_id = module.saint-service.query_api_gateway_resource_id
  mutation_api_parent_id = module.saint-service.mutation_api_gateway_resource_id
    
  function_name = "saints"
  file_name     = "saint"

  depends_on = [
    module.saint-service
   ]
    
  environment = var.environment
  db_host              = var.db_host
  db_user              = var.db_user
  db_password          = var.db_password
  db_name              = var.db_name
}

module "saints_id" {
  source = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//skeleton/services/service-function"
  service_name = var.service_name
  query_api_parent_id = module.saints.query_api_gateway_resource_id
  mutation_api_parent_id = module.saints.mutation_api_gateway_resource_id
  is_auth_required = false
    
  function_name = "saints_id"
  file_name     = "saint"
  path_part     = "{id}"
  depends_on = [
    module.example-service
   ]
    
  environment = var.environment
  db_host              = var.db_host
  db_user              = var.db_user
  db_password          = var.db_password
  db_name              = var.db_name
}
