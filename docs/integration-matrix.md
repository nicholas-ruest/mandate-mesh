# INTEGRATION_MATRIX

| Ingredient | Revision/version/license | Boundary | Owner | Test | Ablation |
|---|---|---|---|---|---|
| RuVector Core | c44d7337 / 2.4.0 / Apache-2.0 | team vector insert/search | mm-ruvector/recall | roundtrip | no recall |
| RVF Runtime | 0.3.2 / Apache-2.0 | contract append/reopen/read | mm-rvf/repository | restart | no durability |
| Microsoft Regorus | 99f794a / 0.13.0 / MIT | mandate input to allow/deny | mm-regorus/policy | allow/deny | no policy |
| Contextual calibration plus SCoRE | arXiv 2604.17950; 2603.24704 | samples to trusted candidate set | mm-capability/calibration | boundaries | no calibration |
