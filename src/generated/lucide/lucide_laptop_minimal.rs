use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "12" rx = "2" ry = "2" x = "3" y = "4" ></ rect > < line x2 = "22" y2 = "20" y1 = "20" x1 = "2" ></ line > < / > } } pub const LUCIDE_LAPTOP_MINIMAL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2")] } ;