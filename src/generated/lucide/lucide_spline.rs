use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "19" cy = "5" r = "2" ></ circle > < circle cy = "19" cx = "5" r = "2" ></ circle > < path d = "M5 17A12 12 0 0 1 17 5" ></ path > < / > } } pub const LucideSpline : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;