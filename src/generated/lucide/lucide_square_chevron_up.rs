use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" y = "3" width = "18" height = "18" rx = "2" ></ rect > < path d = "m8 14 4-4 4 4" ></ path > < / > } } pub const LUCIDE_SQUARE_CHEVRON_UP : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("width" , "24")] } ;