use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" y = "3" height = "18" x = "3" width = "18" ></ rect > < path d = "m16 10-4 4-4-4" ></ path > < / > } } pub const LUCIDE_SQUARE_CHEVRON_DOWN : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24")] } ;