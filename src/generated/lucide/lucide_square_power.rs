use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" width = "18" x = "3" y = "3" height = "18" ></ rect > < path d = "M12 7v5" ></ path > < path d = "M8 9a5.14 5.14 0 0 0 4 8 4.95 4.95 0 0 0 4-8" ></ path > < / > } } pub const LUCIDE_SQUARE_POWER : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;