use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m17 14 3 3.3a1 1 0 0 1-.7 1.7H4.7a1 1 0 0 1-.7-1.7L7 14h-.3a1 1 0 0 1-.7-1.7L9 9h-.2A1 1 0 0 1 8 7.3L12 3l4 4.3a1 1 0 0 1-.8 1.7H15l3 3.3a1 1 0 0 1-.7 1.7H17Z" ></ path > < path d = "M12 22v-3" ></ path > < / > } } pub const LUCIDE_TREE_PINE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;