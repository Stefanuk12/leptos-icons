use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 8V6a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2h-6" ></ path > < path d = "M2 12a9 9 0 0 1 8 8" ></ path > < path d = "M2 16a5 5 0 0 1 4 4" ></ path > < line y1 = "20" x2 = "2.01" x1 = "2" y2 = "20" ></ line > < / > } } pub const LUCIDE_CAST : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2")] } ;