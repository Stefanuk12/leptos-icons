use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "5" height = "8" y = "2" rx = "2" width = "14" ></ rect > < rect height = "8" x = "2" width = "20" y = "14" rx = "2" ></ rect > < path d = "M6 18h2" ></ path > < path d = "M12 18h6" ></ path > < / > } } pub const LUCIDE_COMPUTER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("fill" , "none")] } ;