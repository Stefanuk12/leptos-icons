use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "6" y1 = "6" x2 = "3" x1 = "21" ></ line > < line y1 = "12" x1 = "17" x2 = "7" y2 = "12" ></ line > < line y2 = "18" y1 = "18" x1 = "19" x2 = "5" ></ line > < / > } } pub const LucideAlignCenter : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;