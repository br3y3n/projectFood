#[derive(Debug, Serialize, Deserialize)]
struct Receta {
    id: Option<ObjectId>,
    categoria: String,
    precio: f64,
    descripcion: String,
}
