use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" width = "18" rx = "2" height = "18" x = "3" ></ rect > < path d = "M8 12h8" ></ path > < path d = "m12 16 4-4-4-4" ></ path > < / > } } pub const LUCIDE_SQUARE_ARROW_RIGHT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;