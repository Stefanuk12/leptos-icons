use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "3" y1 = "6" x1 = "21" y2 = "6" ></ line > < line y2 = "12" x2 = "3" y1 = "12" x1 = "15" ></ line > < line y1 = "18" y2 = "18" x2 = "3" x1 = "17" ></ line > < / > } } pub const LucideAlignLeft : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;