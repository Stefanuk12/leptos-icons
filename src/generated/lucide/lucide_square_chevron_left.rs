use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" rx = "2" x = "3" y = "3" height = "18" ></ rect > < path d = "m14 16-4-4 4-4" ></ path > < / > } } pub const LUCIDE_SQUARE_CHEVRON_LEFT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24")] } ;