use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 6h4" ></ path > < path d = "M2 10h4" ></ path > < path d = "M2 14h4" ></ path > < path d = "M2 18h4" ></ path > < rect width = "16" height = "20" rx = "2" y = "2" x = "4" ></ rect > < path d = "M16 2v20" ></ path > < / > } } pub const LUCIDE_NOTEBOOK : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;