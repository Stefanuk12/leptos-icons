use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "12" x = "3" y = "4" ry = "2" rx = "2" width = "18" ></ rect > < line x1 = "2" y2 = "20" x2 = "22" y1 = "20" ></ line > < / > } } pub const LUCIDE_LAPTOP_2 : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;