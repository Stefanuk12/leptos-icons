use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "12" x1 = "4" y2 = "12" x2 = "20" ></ line > < line x1 = "4" y2 = "6" y1 = "6" x2 = "20" ></ line > < line x1 = "4" x2 = "20" y1 = "18" y2 = "18" ></ line > < / > } } pub const LucideMenu : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24")] } ;