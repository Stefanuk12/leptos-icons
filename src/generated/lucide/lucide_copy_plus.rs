use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "15" y1 = "12" y2 = "18" x1 = "15" ></ line > < line y1 = "15" x2 = "18" x1 = "12" y2 = "15" ></ line > < rect x = "8" ry = "2" width = "14" y = "8" height = "14" rx = "2" ></ rect > < path d = "M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" ></ path > < / > } } pub const LUCIDE_COPY_PLUS : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;