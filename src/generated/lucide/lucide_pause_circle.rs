use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cx = "12" cy = "12" ></ circle > < line y1 = "15" x1 = "10" y2 = "9" x2 = "10" ></ line > < line x2 = "14" x1 = "14" y1 = "15" y2 = "9" ></ line > < / > } } pub const LucidePauseCircle : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;