use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "2" width = "20" rx = "5" ry = "5" height = "20" x = "2" ></ rect > < path d = "M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z" ></ path > < line y2 = "6.5" x1 = "17.5" x2 = "17.51" y1 = "6.5" ></ line > < / > } } pub const LUCIDE_INSTAGRAM : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none")] } ;