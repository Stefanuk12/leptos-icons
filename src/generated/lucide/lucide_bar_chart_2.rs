use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "18" y1 = "20" y2 = "10" x2 = "18" ></ line > < line y1 = "20" y2 = "4" x1 = "12" x2 = "12" ></ line > < line x1 = "6" x2 = "6" y1 = "20" y2 = "14" ></ line > < / > } } pub const LucideBarChart2 : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;