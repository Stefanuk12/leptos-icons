use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "18" cy = "5" r = "3" ></ circle > < circle cy = "12" cx = "6" r = "3" ></ circle > < circle cx = "18" cy = "19" r = "3" ></ circle > < line y2 = "17.49" y1 = "13.51" x1 = "8.59" x2 = "15.42" ></ line > < line x1 = "15.41" x2 = "8.59" y1 = "6.51" y2 = "10.49" ></ line > < / > } } pub const LucideShare2 : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2")] } ;