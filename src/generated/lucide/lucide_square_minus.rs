use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" y = "3" x = "3" height = "18" rx = "2" ></ rect > < path d = "M8 12h8" ></ path > < / > } } pub const LUCIDE_SQUARE_MINUS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;