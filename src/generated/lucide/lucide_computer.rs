use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "8" x = "5" y = "2" width = "14" rx = "2" ></ rect > < rect x = "2" width = "20" y = "14" rx = "2" height = "8" ></ rect > < path d = "M6 18h2" ></ path > < path d = "M12 18h6" ></ path > < / > } } pub const LUCIDE_COMPUTER : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;