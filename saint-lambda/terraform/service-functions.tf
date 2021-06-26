module "test2" {
  source = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//skeleton/services/service-function"
  service_name = var.service_name
  query_api_parent_id = module.saint-service.query_api_gateway_resource_id
  mutation_api_parent_id = module.saint-service.mutation_api_gateway_resource_id
    
  function_name = "test2"
  depends_on = [
    module.saint-service
   ]
    
  environment = var.environment
  db_host              = var.db_host
  db_user              = var.db_user
  db_password          = var.db_password
  db_name              = var.db_name
}

module "test3" {
  source = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//skeleton/services/service-function"
  service_name = var.service_name
  query_api_parent_id = module.saint-service.query_api_gateway_resource_id
  mutation_api_parent_id = module.saint-service.mutation_api_gateway_resource_id

  function_name = "test3"
  depends_on = [
    module.saint-service
   ]

  environment = var.environment
  db_host              = var.db_host
  db_user              = var.db_user
  db_password          = var.db_password
  db_name              = var.db_name
}
