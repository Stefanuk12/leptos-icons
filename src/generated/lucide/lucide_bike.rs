use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "18.5" cy = "17.5" r = "3.5" ></ circle > < circle cy = "17.5" r = "3.5" cx = "5.5" ></ circle > < circle cy = "5" r = "1" cx = "15" ></ circle > < path d = "M12 17.5V14l-3-3 4-3 2 3h2" ></ path > < / > } } pub const LUCIDE_BIKE : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;