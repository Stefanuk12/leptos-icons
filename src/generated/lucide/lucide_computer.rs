use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "14" rx = "2" y = "2" x = "5" height = "8" ></ rect > < rect height = "8" rx = "2" width = "20" y = "14" x = "2" ></ rect > < path d = "M6 18h2" ></ path > < path d = "M12 18h6" ></ path > < / > } } pub const LUCIDE_COMPUTER : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;