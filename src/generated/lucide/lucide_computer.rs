use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "8" x = "5" width = "14" y = "2" rx = "2" ></ rect > < rect x = "2" width = "20" height = "8" y = "14" rx = "2" ></ rect > < path d = "M6 18h2" ></ path > < path d = "M12 18h6" ></ path > < / > } } pub const LUCIDE_COMPUTER : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;