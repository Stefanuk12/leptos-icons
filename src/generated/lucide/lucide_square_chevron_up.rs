use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" x = "3" height = "18" width = "18" y = "3" ></ rect > < path d = "m8 14 4-4 4 4" ></ path > < / > } } pub const LUCIDE_SQUARE_CHEVRON_UP : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;