use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "18" x = "3" width = "18" y = "3" ></ rect > < path d = "m14 16-4-4 4-4" ></ path > < / > } } pub const LUCIDE_SQUARE_CHEVRON_LEFT : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24")] } ;