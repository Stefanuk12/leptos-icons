use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "5" r = "3" cx = "18" ></ circle > < circle cy = "12" r = "3" cx = "6" ></ circle > < circle cx = "18" cy = "19" r = "3" ></ circle > < line x1 = "8.59" x2 = "15.42" y1 = "13.51" y2 = "17.49" ></ line > < line y1 = "6.51" x2 = "8.59" x1 = "15.41" y2 = "10.49" ></ line > < / > } } pub const LucideShare2 : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;