use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "6" x1 = "6" y2 = "20" y1 = "4" ></ line > < polygon points = "10,4 20,12 10,20" ></ polygon > < / > } } pub const LucideStepForward : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;