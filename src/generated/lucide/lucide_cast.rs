use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 8V6a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2h-6" ></ path > < path d = "M2 12a9 9 0 0 1 8 8" ></ path > < path d = "M2 16a5 5 0 0 1 4 4" ></ path > < line x2 = "2.01" y2 = "20" y1 = "20" x1 = "2" ></ line > < / > } } pub const LUCIDE_CAST : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;