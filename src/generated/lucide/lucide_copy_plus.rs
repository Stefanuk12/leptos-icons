use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "15" y1 = "12" y2 = "18" x2 = "15" ></ line > < line x2 = "18" x1 = "12" y2 = "15" y1 = "15" ></ line > < rect rx = "2" width = "14" x = "8" y = "8" height = "14" ry = "2" ></ rect > < path d = "M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" ></ path > < / > } } pub const LUCIDE_COPY_PLUS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24")] } ;