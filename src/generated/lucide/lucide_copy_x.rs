use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "12" x1 = "12" x2 = "18" y2 = "18" ></ line > < line y1 = "18" x1 = "12" x2 = "18" y2 = "12" ></ line > < rect ry = "2" y = "8" x = "8" height = "14" rx = "2" width = "14" ></ rect > < path d = "M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" ></ path > < / > } } pub const LUCIDE_COPY_X : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24")] } ;