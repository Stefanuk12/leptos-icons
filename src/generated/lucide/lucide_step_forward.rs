use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "6" y1 = "4" x1 = "6" y2 = "20" ></ line > < polygon points = "10,4 20,12 10,20" ></ polygon > < / > } } pub const LucideStepForward : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;