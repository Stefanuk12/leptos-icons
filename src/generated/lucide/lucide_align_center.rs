use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "3" x1 = "21" y1 = "6" y2 = "6" ></ line > < line y2 = "12" x1 = "17" x2 = "7" y1 = "12" ></ line > < line y2 = "18" x2 = "5" x1 = "19" y1 = "18" ></ line > < / > } } pub const LucideAlignCenter : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2")] } ;