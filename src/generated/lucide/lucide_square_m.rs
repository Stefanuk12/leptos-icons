use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "18" y = "3" rx = "2" x = "3" ></ rect > < path d = "M8 16V8l4 4 4-4v8" ></ path > < / > } } pub const LUCIDE_SQUARE_M : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2")] } ;