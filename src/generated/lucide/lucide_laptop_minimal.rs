use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" ry = "2" width = "18" y = "4" height = "12" x = "3" ></ rect > < line x1 = "2" x2 = "22" y2 = "20" y1 = "20" ></ line > < / > } } pub const LUCIDE_LAPTOP_MINIMAL : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2")] } ;