use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 5a3 3 0 1 1-6 0 3 3 0 0 1 6 0ZM2.046 15.253c-.058.468.172.92.57 1.175A9.953 9.953 0 0 0 8 18c1.982 0 3.83-.578 5.384-1.573.398-.254.628-.707.57-1.175a6.001 6.001 0 0 0-11.908 0ZM12.75 7.75a.75.75 0 0 0 0 1.5h5.5a.75.75 0 0 0 0-1.5h-5.5Z" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_USER_MINUS : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("aria-hidden" , "true") , ("viewBox" , "0 0 20 20") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "currentColor")] } ;