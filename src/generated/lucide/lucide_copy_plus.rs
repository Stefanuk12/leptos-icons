use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "15" y2 = "18" y1 = "12" x1 = "15" ></ line > < line x2 = "18" y1 = "15" y2 = "15" x1 = "12" ></ line > < rect ry = "2" x = "8" rx = "2" width = "14" height = "14" y = "8" ></ rect > < path d = "M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" ></ path > < / > } } pub const LUCIDE_COPY_PLUS : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;