use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path clip - rule = "evenodd" fill - rule = "evenodd" d = "M17.768 7.793a.75.75 0 0 1-1.06-.025L12.75 3.622v10.003a5.375 5.375 0 0 1-10.75 0V10.75a.75.75 0 0 1 1.5 0v2.875a3.875 3.875 0 0 0 7.75 0V3.622L7.293 7.768a.75.75 0 0 1-1.086-1.036l5.25-5.5a.75.75 0 0 1 1.085 0l5.25 5.5a.75.75 0 0 1-.024 1.06Z" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_ARROW_UTURN_UP : Path = Path { path : icon_path , props : & [("fill" , "currentColor") , ("viewBox" , "0 0 20 20") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("aria-hidden" , "true")] } ;