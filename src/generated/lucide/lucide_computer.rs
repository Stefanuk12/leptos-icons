use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "14" height = "8" x = "5" y = "2" rx = "2" ></ rect > < rect width = "20" rx = "2" y = "14" x = "2" height = "8" ></ rect > < path d = "M6 18h2" ></ path > < path d = "M12 18h6" ></ path > < / > } } pub const LUCIDE_COMPUTER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;