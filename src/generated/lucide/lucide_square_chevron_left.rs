use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" y = "3" x = "3" height = "18" rx = "2" ></ rect > < path d = "m14 16-4-4 4-4" ></ path > < / > } } pub const LUCIDE_SQUARE_CHEVRON_LEFT : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;