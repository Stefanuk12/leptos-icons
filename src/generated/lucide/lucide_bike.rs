use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3.5" cx = "18.5" cy = "17.5" ></ circle > < circle cx = "5.5" r = "3.5" cy = "17.5" ></ circle > < circle cy = "5" cx = "15" r = "1" ></ circle > < path d = "M12 17.5V14l-3-3 4-3 2 3h2" ></ path > < / > } } pub const LUCIDE_BIKE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;