use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "2" ry = "5" width = "20" x = "2" height = "20" rx = "5" ></ rect > < path d = "M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z" ></ path > < line x2 = "17.51" y1 = "6.5" y2 = "6.5" x1 = "17.5" ></ line > < / > } } pub const LUCIDE_INSTAGRAM : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;