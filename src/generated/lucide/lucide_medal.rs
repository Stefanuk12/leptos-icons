use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M7.21 15 2.66 7.14a2 2 0 0 1 .13-2.2L4.4 2.8A2 2 0 0 1 6 2h12a2 2 0 0 1 1.6.8l1.6 2.14a2 2 0 0 1 .14 2.2L16.79 15" ></ path > < path d = "M11 12 5.12 2.2" ></ path > < path d = "m13 12 5.88-9.8" ></ path > < path d = "M8 7h8" ></ path > < circle r = "5" cx = "12" cy = "17" ></ circle > < path d = "M12 18v-2h-.5" ></ path > < / > } } pub const LUCIDE_MEDAL : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;