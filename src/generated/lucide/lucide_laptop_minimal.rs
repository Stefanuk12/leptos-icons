use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "4" width = "18" x = "3" rx = "2" height = "12" ry = "2" ></ rect > < line x2 = "22" y1 = "20" x1 = "2" y2 = "20" ></ line > < / > } } pub const LUCIDE_LAPTOP_MINIMAL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2")] } ;