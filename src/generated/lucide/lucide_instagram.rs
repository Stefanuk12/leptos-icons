use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "5" height = "20" ry = "5" x = "2" y = "2" width = "20" ></ rect > < path d = "M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z" ></ path > < line x2 = "17.51" x1 = "17.5" y1 = "6.5" y2 = "6.5" ></ line > < / > } } pub const LUCIDE_INSTAGRAM : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;