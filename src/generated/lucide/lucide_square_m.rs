use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "18" y = "3" rx = "2" x = "3" ></ rect > < path d = "M8 16V8l4 4 4-4v8" ></ path > < / > } } pub const LUCIDE_SQUARE_M : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("width" , "24")] } ;