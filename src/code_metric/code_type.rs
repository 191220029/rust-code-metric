use std::fmt::Display;

// pub enum CodeType {
//     Function {
//         label: String,
//         lines: u32,
//         params: u32,
//         calls: u32,
//     },
//     // FnCall {
//     //     label: String,
//     //     times: u32,
//     // },
//     Method {
//         label: String,
//         lines: u32,
//         params: u32,
//         calls: u32
//     },
//     // MethodCall{
//     //     label: String,
//     // },
//     Struct {
//         label: String,
//         members: u32,
//         methods: u32
//     },
//     Trait {
//         label: String,
//         methods: u32,
//     },
//     Dyn,
// }

// impl Display for CodeType {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             CodeType::Function { label, lines, params, calls } => 
//                 write!(f, "Function {}: lines={}, params={}, calls={}", label, lines, params, calls),
//             CodeType::Method { label, lines, params, calls } => 
//                 write!(f, "Method {}: lines={}, params={}, calls={}", label, lines, params, calls),
//             CodeType::Struct { label, members, methods } => 
//                 write!(f, "Struct {}: members={}, methods={}", label, members, methods),
//             CodeType::Trait { label, methods } => 
//                 write!(f, "Trait {}: methods={}", label, methods),
//             CodeType::Dyn => write!(f, "Dyn use"),
//         } 
//     }
// }

