use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "3" y1 = "6" y2 = "6" x1 = "21" ></ line > < line x1 = "17" y1 = "12" x2 = "7" y2 = "12" ></ line > < line y1 = "18" x1 = "19" x2 = "5" y2 = "18" ></ line > < / > } } pub const LucideAlignCenter : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;