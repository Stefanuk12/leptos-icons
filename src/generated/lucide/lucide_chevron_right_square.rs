use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" height = "18" y = "3" rx = "2" width = "18" ></ rect > < path d = "m10 8 4 4-4 4" ></ path > < / > } } pub const LUCIDE_CHEVRON_RIGHT_SQUARE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;