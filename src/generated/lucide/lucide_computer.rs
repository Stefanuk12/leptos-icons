use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "14" height = "8" x = "5" y = "2" rx = "2" ></ rect > < rect y = "14" rx = "2" width = "20" height = "8" x = "2" ></ rect > < path d = "M6 18h2" ></ path > < path d = "M12 18h6" ></ path > < / > } } pub const LUCIDE_COMPUTER : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;